# SAM_Lambda_DynDB
This will be a project designed to develop a CRUD capable lamda function in Rust which then links to DynamoDB. The goal here is to develop and test this locally via the SAM CLI and then deploy via the SAM template to generate the cloudformation stack. This project should provide insights into developing with the Rust language as well as more familiarity with serverless development.

## Step by Step guide:

I am utilizing github codespaces for convenience but gitlab's gitpods are also good or your local environment. I enjoy using cloud based developer environment because each project is isolated and I don't have to worry about the state of my local machine. I will include the step by step guide here:

To begin, I am going to pull some template files from a past project which include a baseline CI/CD yml and a Makefile, when I make changes to these document I will indicate those changes here.

Of note will be the working directory environment variable in my yml file and the makefile, in its current state, should be located at the same level as the Cargo.toml file when it is created with the AWS SAM CLI.