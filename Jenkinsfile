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

cargo build'''
      }
    }

    stage('Save') {
      steps {
        archiveArtifacts(allowEmptyArchive: true, artifacts: 'rasis/target')
      }
    }

  }
  environment {
    ENV_TEST = 'TEST'
  }
}