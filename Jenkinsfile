pipeline {
  agent {
    dockerfile {
      filename './Dockerfiles/rust-builder-aarch64/Dockerfile'
      args '--privileged'
    }

  }
  stages {
    stage('setup') {
      steps {
        sh '''rustc --version
        echo $HOME
service docker start
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


'''
      }
    }

  }
  environment {
    ENV_TEST = 'TEST'
  }
}