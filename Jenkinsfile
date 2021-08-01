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