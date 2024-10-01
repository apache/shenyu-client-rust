#!/bin/bash

cd ~
git clone https://github.com/apache/shenyu
cd ~/shenyu
mvn clean -Prelease -Dmaven.javadoc.skip=true -B -Drat.skip=true -Djacoco.skip=true -DskipITs -DskipTests package -pl ./shenyu-dist/shenyu-admin-dist -am -U
cd ~/shenyu/shenyu-dist/shenyu-admin-dist/target
tar -xzf apache-shenyu-*.tar.gz
sh ~/shenyu/shenyu-dist/shenyu-admin-dist/target/apache-shenyu-*/bin/start.sh
