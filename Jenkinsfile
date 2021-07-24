pipeline {
  agent {
    docker {
      image 'rust:alpine3.12'
    }

  }
  stages {
    stage('setup') {
      steps {
        sh '''rustc --version

ls
'''
      }
    }

  }
  environment {
    ENV_TEST = 'TEST'
  }
}