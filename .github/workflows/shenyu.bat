powershell -Command "Invoke-WebRequest -Uri 'https://github.com/libarchive/libarchive/releases/download/v3.7.5/libarchive-3.7.5.zip' -OutFile 'tar.zip'"
powershell -Command "Expand-Archive -Path 'tar.zip' -DestinationPath '%GITHUB_WORKSPACE%\tar'"
powershell -Command "Add-Content -Path ‘%GITHUB_PATH%’ -Value '%GITHUB_WORKSPACE%\tar\bin'"
tar --version

git config --system core.longpaths true
git clone https://github.com/apache/shenyu
cd shenyu/shenyu-dist/shenyu-admin-dist
set pomFile="pom.xml"
powershell -Command "(Get-Content %pomFile%) -replace '<finalName>.*</finalName>', '<finalName>shenyu-admin</finalName>' | Set-Content %pomFile%"
cd ../../
mvn clean -Prelease -Dmaven.javadoc.skip=true -B -Drat.skip=true -Djacoco.skip=true -DskipITs -DskipTests package -pl ./shenyu-dist/shenyu-admin-dist -am -U
cd shenyu-dist\shenyu-admin-dist\target
tar -zxvf shenyu-admin-admin-bin.tar.gz
cd shenyu-admin-admin-bin\bin
.\start.bat
