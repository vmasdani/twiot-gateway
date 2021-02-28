rm -rf dist &&\
mkdir -p dist/frontend &&\
cross build --target armv7-unknown-linux-musleabihf &&\
cp target/armv7-unknown-linux-musleabihf/debug/twiot-gateway dist &&\
cd frontend &&\
./build.sh &&\
cd ../ &&\
cp -r frontend/dist/* dist/frontend &&\
cd dist &&\
zip -r release.zip * &&\
cd ..
