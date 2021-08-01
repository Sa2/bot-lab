pipeline {
  agent any
  // agent {
  //   dockerfile {
  //     filename './Dockerfiles/rust-builder-aarch64/Dockerfile'
  //     // args '-u root:sudo --privileged'
  //   }
  // }

  stages {
//     stage('setup') {
//       steps {
//         sh '''rustc --version
// '''
//       }
//     }

    stage('build') {
      agent {
        docker {
          label 'master'
          image 'rust:1.53-buster'
        }
      }
      steps {
        sh '''cd rasis
cargo build --release'''
      }
    }

//     stage('Save') {
//       when {
//         expression {
//           env.BRANCH_NAME.contains("master")
//         }
//       }
//       steps {
//         sh '''ls -l rasis/target/release

// '''
//         archiveArtifacts 'rasis/target/release/rasis'
//       }
//     }
    stage('Containerize') {
      when {
        expression {
          env.BRANCH_NAME.contains("master")
        }
      }
      steps {
        sh '''docker build -t bot-lab-rasis:latest .

'''
        post {
          always {
            deleteDir()
          }
        }
      }
    }
  }
  environment {
    ENV_TEST = 'TEST'
  }
}