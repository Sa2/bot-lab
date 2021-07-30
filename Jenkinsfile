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
sleep(5)
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