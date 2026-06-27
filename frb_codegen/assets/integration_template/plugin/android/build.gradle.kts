plugins {
    id("com.android.library")
}

group = "com.flutter_rust_bridge.REPLACE_ME_DART_PACKAGE_NAME"
version = "1.0"

repositories {
    google()
    mavenCentral()
}

android {
    namespace = "com.flutter_rust_bridge.REPLACE_ME_DART_PACKAGE_NAME"

    compileSdk = 37

    // Uses the NDK version supplied by the consuming Flutter app.
    ndkVersion = android.ndkVersion

    defaultConfig {
        minSdk = 24
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }
}

apply(from = "../cargokit/gradle/plugin.gradle")

(extensions.getByName("cargokit") as groovy.lang.GroovyObject).apply {
    setProperty("manifestDir", "../REPLACE_ME_RUST_CRATE_DIR")
    setProperty("libname", "REPLACE_ME_RUST_CRATE_NAME")
}