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

    stage('build') {
      steps {
        sh '''cd rasis

cargo build'''
      }
    }

  }
  environment {
    ENV_TEST = 'TEST'
  }
}