rm(list =ls());gc()
options(repos = "https://cloud.r-project.org/")
library(binancer)
library(TTR)
library(quantmod)
library(PerformanceAnalytics)
library(xgboost)
library(ROCR)

con <- binance_klines("BTCUSDT", interval = "8h", start_time = Sys.time()-(60*60*24*31*6), end_time = Sys.time())

# Get OHLC
con_OHLC <- as.xts(OHLC(con))

close_price <- Cl(con_OHLC)
sma <- SMA(close_price)
ema <- EMA(close_price)
bb <- BBands(close_price)
mom <- momentum(close_price)
roc <- ROC(close_price)
macd <- MACD(close_price)
rsi <- RSI(close_price)


r <- close_price - Lag(close_price) # nominal price change
delta <-0.015 # fee 
signal <-c(0) # first date has no signal

#Loop over all trading days (except the first)
for (i in 2: length(close_price)){
  if (r[i] > delta){
    signal[i]<- 1
  } else 
    signal[i]<- 0
}

signal <- reclass(signal,as.xts(con))

#daily return
ret<-dailyReturn(as.xts(con_OHLC))*signal

# Model the signals with TA indicators using XGboost algo

signal_with_TA <- data.frame(signal, sma, ema, bb, rsi, sma)
reclass(signal_with_TA,as.xts(con))


# We split data based on time (0.8 of sample size goes to train)
train_index <- seq(0.8*nrow(signal_with_TA))

train <- signal_with_TA[train_index,]
summary(as.POSIXct(rownames(train)))

test <- signal_with_TA[-train_index,] 
summary(as.POSIXct(rownames(test))) # a good month of 8h close price

xgb_train <- xgb.DMatrix(data = as.matrix(train[,-1]), 
                    label = train$signal) #400 observations
xgb_test <- xgb.DMatrix(data = as.matrix(test[,-1]), 
                    label = test$signal) #100 observations

# Set a seed for reproducibility
set.seed(123)
cv <- xgb.cv(data = xgb_train, nfold = 5,
             objective = "binary:logistic", 
             nrounds = 100, 
             early_stopping_rounds = 10,
             maximize = FALSE, verbose = FALSE)

# Get optimal number of rounds
nrounds <- which.min(cv$evaluation_log$test_logloss_mean)

# Train XGBoost model with optimal hyperparameters
bst_select <- xgboost(data = xgb_train, 
                      nrounds = nrounds,
                      max_depth = 3, 
                      eta = 0.1, 
                      objective = "binary:logistic", 
                      verbose = FALSE)


importance <- xgb.importance(feature_names = colnames(xgb_train), model = bst_select)


# Create a logistic regression based on most important TA indicators we retrieved from XGboost
formula_str <- paste("signal ~", paste(importance$Feature, collapse = " + "))
model_formula  <- as.formula(formula_str)

  
importance_model <- glm(formula = model_formula,
                        data = train,
                        family = binomial(link = "logit"))

# Compute feature importance
predictions <- predict(importance_model, test,
                  type = 'response')

# inpute 0 trading signal if NA -- ## big problem if first 4 predictions are 0 by default lol
predictions <-ifelse(is.na(predictions) ,0, predictions )

pred <- pROC::roc(test[["signal"]], predictions)
cat(pred$auc)
#0.6 - quite poor performance

# Find optimal threshold for best accuracy
optimal_threshold <- pROC::coords(pred, "best", ret = "threshold", input.sort = FALSE)


confusion_matrix <- table(predictions > optimal_threshold[[1]], signal[-train_index])

# Find model accuracy:
cat("accuracy:" , sum(diag(confusion_matrix)) / sum(confusion_matrix))

### cumulative returns

# Predicted_signal on test set
predicted_signal <- ifelse(predictions > optimal_threshold[[1]], 1,0)

# reclass to xts for return plot
predicted_signal <- reclass(predicted_signal, as.xts(test))

ret_importance_model <- dailyReturn(as.xts(test))*predicted_signal

# Replace infinite values with NA
ret_importance_model[!is.finite(ret_importance_model)] <- NA

print(ret_importance_model)

result <- "hello" 

cat(result)
