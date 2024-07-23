terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region     = "us-west-2"
}

resource "aws_instance" "app_server" {
  ami           = "ami-074be47313f84fa38"
  instance_type = "t2.micro"

  user_data = <<-EOF
              #!/bin/bash
              yum update -y
              yum install -y docker
              service docker start
              usermod -a -G docker ec2-user
              docker network create net-bot
              docker run -d -p 6379:6379 -p 8001:8001 --name redis-bot --network net-bot redis/redis-stack
              docker run -d --name bot-scythe --network net-bot alexroza/bot-scythe-marie-curie

              EOF

  tags = {
    Name = "ExampleAppServerInstance"
  }
}
