pipeline {
  agent {
    dockerfile {
      filename './Dockerfiles/rust-builder-aarch64/Dockerfile'
      // args '-u root:sudo --privileged'
    }
  }

  triggers {
    pollSCM('H * * * *')
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
  }
  environment {
    ENV_TEST = 'TEST'
  }
}