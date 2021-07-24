pipeline {
  agent {
    dockerfile {
      filename 'Dockerfile'
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
        sh 'ls rasis/target/release'
        archiveArtifacts(allowEmptyArchive: true, artifacts: 'rasis/target')
      }
    }

  }
  environment {
    ENV_TEST = 'TEST'
  }
}