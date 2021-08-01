pipeline {
  agent {
    dockerfile {
      filename './Dockerfiles/rust-builder-aarch64/Dockerfile'
      // args '-u root:sudo --privileged'
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
      when {
        expression {
          env.BRANCH_NAME.contains("master")
        }
      }
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