plugins {
    kotlin("jvm") version "2.1.10"
}

group = "com.suisdks"
version = "0.1.0"

kotlin {
    jvmToolchain(17)
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.jetbrains.kotlin:kotlin-stdlib")

    implementation("io.grpc:grpc-netty-shaded:1.76.0")
    implementation("io.grpc:grpc-protobuf:1.76.0")
    implementation("io.grpc:grpc-stub:1.76.0")
    implementation("io.grpc:grpc-auth:1.76.0")
    implementation("com.google.protobuf:protobuf-java-util:4.33.0")
    implementation("com.google.code.gson:gson:2.11.0")
    implementation("com.google.crypto.tink:tink:1.13.0")
    implementation("com.graphql-java:graphql-java:22.3")

    implementation("org.bouncycastle:bcprov-jdk18on:1.82")

    testImplementation(kotlin("test"))
    testImplementation("io.grpc:grpc-testing:1.76.0")
    testImplementation("org.junit.jupiter:junit-jupiter:5.11.4")
}

tasks.test {
    useJUnitPlatform()
    jvmArgs("--add-modules", "jdk.httpserver")
}
