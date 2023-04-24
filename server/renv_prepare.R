# Make sure tcltk
install.packages("tcltk")
install.packages("renv")

# Initialize the project and create a project-specific library
renv::init()

# Install the packages required by your script
renv::install(c("RSQLite"))

# Save a snapshot of the current environment to renv.lock
renv::snapshot()
