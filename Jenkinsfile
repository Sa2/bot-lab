pipeline {
  agent any
  stages {
    stage('build') {
      agent {
        docker {
          image 'rust:1.53-buster'
        }

      }
      steps {
        sh '''cd rasis
cargo build --release'''
      }
    }

    stage('Containerize') {
      when {
        expression {
          env.BRANCH_NAME.contains("main")
        }

      }
      steps {
        sh '''pwd
ls ./scripts/start.sh
ls -la ./rasis
ls ./rasis/credentials/bot-lab/env.sh
ls ./rasis/target/release/rasis
docker build -t bot-lab-rasis:latest .

'''
        post() {
          always() {
            deleteDir()
          }

        }

      }
    }

  }
  environment {
    ENV_TEST = 'TEST'
  }
}