plugins {
    `com.android.application`
}

android {
	compileSdk(33)
	ndkVersion('25.1.8937393')

	defaultConfig {
		applicationId = 'me.ohsey.android'
		minSdkVersion(14)
		targetSdkVersion(33)
		externalNativeBuild {
			cmake {
				arguments('-DANDROID_STL=c++_static')
			}
		}
	}
	buildTypes {
		release {
			minifyEnabled(false)
			proguardFiles(getDefaultProguardFile('proguard-android.txt'), 'proguard-rules.pro')
		}
	}
	externalNativeBuild {
		cmake {
			path('src/main/cpp/CMakeLists.txt')
		}
	}
	namespace('me.ohsey.android')
}

dependencies {
	implementation fileTree(dir: 'libs', include: ['*.jar'])
	implementation 'androidx.appcompat:appcompat:1.5.1'
	implementation 'androidx.constraintlayout:constraintlayout:2.1.4'
}

