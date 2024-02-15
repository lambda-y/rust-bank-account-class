terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.6"
    }
  }
}

provider "aws" {
  region = "us-east-1" # Replace with your desired region
}

# S3 bucket to store the Lambda zip file
resource "aws_s3_bucket" "lambda_code" {
  bucket = "my-lambda-code-bucket"

  acl    = "private"

  versioning {
    enabled = true
  }
}

# IAM role for the Lambda function
resource "aws_iam_role" "lambda_role" {
  name = "my-lambda-role"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF

  managed_policy_arns = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  ]
}

# IAM policy for the Lambda function
resource "aws_iam_policy" "lambda_policy" {
  name = "my-lambda-policy"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Resource": [
        "*"
      ],
      "Action": [
        "logs:PutLogEvents",
        "s3:GetObject",
        "xray:PutTraceSegments",
        "xray:PutTelemetryEvents"
      ]
    }
  ]
}
EOF
}

# Attach policy to role
resource "aws_iam_role_policy_attachment" "lambda_role_policy" {
  role       = aws_iam_role.lambda_role.name
  policy_arn = aws_iam_policy.lambda_policy.arn
}

# Lambda function
resource "aws_lambda_function" "my_rust_lambda" {
  filename         = "path/to/your/lambda.zip" # Replace with your zip file path
  function_name    = "my-rust-lambda"
  role             = aws_iam_role.lambda_role.arn
  handler          = "handler_function::handler" # Replace with your handler function
  runtime         = "provided.al2" # For Rust 1.52 and up
  #runtime         = "provided.al2x" # For Rust 1.60 and up (experimental)
  memory_size     = 128
  timeout         = 30
  source_code_hash = filebase64sha256(filename)

  environment {
    variables = {
      # Add any environment variables your Lambda needs
    }
  }
}

# API Gateway configuration (optional)
# ...

# CloudWatch Logs configuration (optional)
# ...
