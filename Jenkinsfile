pipeline {
  agent {
    dockerfile {
      filename './Dockerfiles/rust-builder-aarch64/Dockerfile'
    }

  }
  stages {
    stage('setup') {
      steps {
        sh '''rustc --version
'''
      }
    }

    stage('build') {
      steps {
        sh '''cd rasis

cargo build --release'''
      }
    }

    stage('Save') {
      steps {
        sh '''ls -l rasis/target/release

'''
        archiveArtifacts 'rasis/target/release/rasis'
      }
    }

    stage('Containerize') {
      agent any
      steps {
        sh '''ls -l
ls -l rasis

# docker build -t rasis-bot:latest ./Dockerfiles/runtime/Dockerfile
'''
      }
    }

  }
  environment {
    ENV_TEST = 'TEST'
  }
}