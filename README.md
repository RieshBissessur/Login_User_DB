# Login_User_DB
A Rust database project with user registraction, login and the abilty to reset your password

This is dependant on RUST being installed

Rust can be installed by following these instructions:

	https://www.rust-lang.org/tools/install

Or on Mac OS X using this curl command:

	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 

<br>

Building the Console Application
=====================================================================================================================================================================
To build or run a new exectuable Rust must be installed.
- ## Build an executable
    - In the project directory run the following commands to build to the target directory from which you can run the executable file.
  
  - ``` cargo build ```
  <br>

- ## Compile and run the console application
  - In the project directory run the following commands to compile and run the console application.
 
  - ```  cargo run ```

<br>


Server Requests
=====================================================================================================================================================================

## Get Requests
- ## General requests
  - Health check: {URl}:{Port}/health
    - Responds with a 200 to show the server is healthy 

## Post Requests
- ### Register
  - Create an account by sending account details: {URl}:{Port}/register
    - Json body for post contains a username, email and a password as strings

- ### Login
  - Login and create a session key by sending username and password: {URl}:{Port}/login
    - Json body for the post contains a username/email and password as strings and a version as float

- ### Get User Data
  - Reteive user data by sending username and session key: {URl}:{Port}/user_data
    - Json body for the post contains a username, password as strings

- ### Request Password Reset
  - Request a password reset by sending an email address: {URl}:{Port}/reset_request
    - Json body for the post contains an email address

- ### Submit OTP and new password
  - Reteive user data by sending the email, otp recieved and new password: {URl}:{Port}/check_otp
    - Json body for the post contains a email, otp recieved and new password as strings 