# OpenIAP Java Client

To build the project:
```bash
mvn clean package
```

in pom.xml, add:
```
io.openiap:client:0.0.38
```

or for Gradle uses, add:
```bash
dependencies {
    implementation 'io.openiap:client:0.0.38'
}
```

To run the test application:
```bash
mvn package
java -jar target/client-0.0.38-jar-with-dependencies.jar
```

