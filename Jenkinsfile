pipeline {
  agent {
    docker {
      image 'rust:1.53-buster'
    }

  }
  stages {
    stage('setup') {
      steps {
        sh '''rustc --version

apt update
apt install -y gcc-aarch64-linux-gnu'''
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