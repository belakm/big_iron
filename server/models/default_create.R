# Load required libraries
library(DBI)
library(dplyr)
library(caret)

# Connect to the SQLite database
con <- dbConnect(RSQLite::SQLite(), dbname = "database.sqlite")

# Load klines into a data frame
klines <- dbGetQuery(con, "SELECT * FROM klines")

# Convert timestamp columns from integer to POSIXct
klines$open_time <- as.POSIXct(klines$open_time, origin = "1970-01-01", tz = "UTC")
klines$close_time <- as.POSIXct(klines$close_time, origin = "1970-01-01", tz = "UTC")

# Split klines into training and testing sets
train_index <- 1:floor(0.8 * nrow(klines))
train_set <- klines[train_index, ]
test_set <- klines[-train_index, ]

# Define the target variable and the features
target <- "close"
features <- c("open", "high", "low", "volume")

# Define the training control
train_control <- trainControl(method = "cv", number = 5)

# Train the model using the training set
model <- train(train_set[, features], train_set[, target], method = "glmnet", trControl = train_control)

# Test the model using the testing set
predictions <- predict(model, test_set[, features])

# Compute the accuracy of the model
accuracy <- mean(predictions == test_set[, target])

# Save the model to a file
saveRDS(model, file = "/models/prediction.rds")
