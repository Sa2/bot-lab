pipeline {
  agent {
    dockerfile {
      filename './Dockerfiles/rust-builder-aarch64/Dockerfile'
      args '-u root:sudo --privileged'
    }

  }
  stages {
    stage('setup') {
      steps {
        sh '''rustc --version
service docker start
sleep 5
docker version
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
      steps {
        sh '''ls -l
ls -l rasis
pwd

ls /Dockerfiles/runtime/start.sh
ls /rasis/target/release/rasis
ls /rasis/credentials/bot-lab/env.sh

docker build -t rasis-bot:test ./Dockerfiles/runtime
'''
      }
    }

  }
  environment {
    ENV_TEST = 'TEST'
  }
}