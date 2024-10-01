powershell -Command "Invoke-WebRequest -Uri 'https://github.com/apache/shenyu/archive/refs/tags/v2.6.1.zip' -OutFile 'shenyu.zip'"
powershell -Command "Expand-Archive -Path 'shenyu.zip' -DestinationPath '.\shenyu'"

cd shenyu/shenyu-2.6.1/shenyu-dist/shenyu-admin-dist
set pomFile="pom.xml"
powershell -Command "(Get-Content %pomFile%) -replace '<finalName>.*</finalName>', '<finalName>shenyu-admin</finalName>' | Set-Content %pomFile%"

cd src/main/assembly/
set pomFile="binary.xml"
powershell -Command "(Get-Content %pomFile%) -replace '<format>.*</format>', '<format>zip</format>' | Set-Content %pomFile%"

cd ../../../../../
mvn clean -Prelease -Dmaven.javadoc.skip=true -B -Drat.skip=true -Djacoco.skip=true -DskipITs -DskipTests package -pl ./shenyu-dist/shenyu-admin-dist -am
