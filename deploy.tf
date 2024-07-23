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
  region = "us-west-2"
}

variable "replace_instance" {
  description = "Set to true to replace the EC2 instance"
  type        = bool
  default     = false
}

# Data source to find existing instance by tag
data "aws_instances" "existing_instances" {
  filter {
    name   = "tag:Name"
    values = ["ExampleAppServerInstance"]
  }

  filter {
    name   = "instance-state-name"
    values = ["running"]
  }
}

resource "null_resource" "shutdown_old_instance" {
  count = var.replace_instance && length(data.aws_instances.existing_instances.ids) > 0 ? 1 : 0

  provisioner "local-exec" {
    command = "aws ec2 stop-instances --instance-ids ${data.aws_instances.existing_instances.ids[0]}"
  }

  triggers = {
    always_run = timestamp()
  }
}

resource "aws_instance" "app_server" {
  count         = var.replace_instance ? 1 : 0
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

  depends_on = [null_resource.shutdown_old_instance]
}
