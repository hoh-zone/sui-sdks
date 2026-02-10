plugins {
    java
}

group = "com.suisdks"
version = "0.1.0"

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("io.grpc:grpc-netty-shaded:1.76.0")
    implementation("io.grpc:grpc-protobuf:1.76.0")
    implementation("io.grpc:grpc-stub:1.76.0")
    implementation("com.google.protobuf:protobuf-java-util:4.33.0")
    implementation("com.google.code.gson:gson:2.11.0")

    implementation("org.bouncycastle:bcprov-jdk18on:1.82")

    testImplementation("io.grpc:grpc-testing:1.76.0")
    testImplementation("org.junit.jupiter:junit-jupiter:5.11.4")
}

tasks.test {
    useJUnitPlatform()
}
