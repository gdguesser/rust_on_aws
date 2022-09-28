## To generate the zip file used to upload to AWS Lambda, we need to compile using the 
following command:
cargo-lambda lambda build --release --arm64 --output-format zip

To see more details please refer to the rust lambda runtime documentation:
https://github.com/awslabs/aws-lambda-rust-runtime#12-build-your-lambda-functions
