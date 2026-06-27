import com.android.build.api.dsl.LibraryExtension

group = "com.flutter_rust_bridge.REPLACE_ME_RUST_CRATE_NAME"
version = "1.0"

buildscript {
    repositories {
        google()
        mavenCentral()
        maven("https://storage.googleapis.com/download.flutter.io")
    }

    dependencies {
        // The Android Gradle Plugin knows how to build native code with the NDK.
        classpath("com.android.tools.build:gradle:9.2.1")
    }
}

allprojects {
    repositories {
        google()
        mavenCentral()
        maven("https://storage.googleapis.com/download.flutter.io")
    }
}

apply(plugin = "com.android.library")

configure<LibraryExtension> {
    namespace = "com.flutter_rust_bridge.REPLACE_ME_RUST_CRATE_NAME"

    // Bumping the plugin compileSdk requires all clients of this plugin
    // to bump the version in their app.
    compileSdk = 37

    // Use the NDK version declared in /android/app/build.gradle.kts
    // of the Flutter project. Replace it with a version number if needed.
    ndkVersion = "30.0.14904198"

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }

    defaultConfig {
        minSdk = 24
    }
}

apply(from = "../cargokit/gradle/plugin.gradle.kts")

extensions.getByName("cargokit").apply {
    javaClass.getMethod("setManifestDir", String::class.java).invoke(this, "../../REPLACE_ME_RUST_CRATE_DIR")
    javaClass.getMethod("setLibname", String::class.java).invoke(this, "REPLACE_ME_RUST_CRATE_NAME")
}
