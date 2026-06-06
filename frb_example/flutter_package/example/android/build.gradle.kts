allprojects {
    repositories {
        google()
        mavenCentral()
    }
}

val newBuildDir: Directory =
    rootProject.layout.buildDirectory
        .dir("../../build")
        .get()
rootProject.layout.buildDirectory.value(newBuildDir)

subprojects {
    configurations.configureEach {
        resolutionStrategy.force(
            "androidx.test:runner:1.6.2",
            "androidx.test:rules:1.6.1",
            "androidx.test:monitor:1.7.2",
            "androidx.test.espresso:espresso-core:3.6.1",
            "androidx.test.espresso:espresso-idling-resource:3.6.1",
        )
    }

    val newSubprojectBuildDir: Directory = newBuildDir.dir(project.name)
    project.layout.buildDirectory.value(newSubprojectBuildDir)
}
subprojects {
    project.evaluationDependsOn(":app")
}

tasks.register<Delete>("clean") {
    delete(rootProject.layout.buildDirectory)
}
