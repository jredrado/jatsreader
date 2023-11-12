#/bin/sh

supervisor () {
    while true; do
        nohup slight -c $2 run $3
        sleep 5
    done
}



ROOT='/workspace/jatsreader/src/slight'
STREAMERPORT=3000
VERSION='release'
SLEEPTIME=1

STREAMER_API_URL=https://$STREAMERPORT-$HOSTNAME.$GITPOD_WORKSPACE_CLUSTER_HOST

echo $STREAMER_API_URL

slight -c $ROOT/storage/main/slightfile.toml secret -k "INSTANCE" -v "storage_1"
slight -c $ROOT/storage/main/slightfile2.toml secret -k "INSTANCE" -v "storage_2"

slight -c $ROOT/register/main/slightfile.toml secret -k "INSTANCE" -v "register_1"

slight -c $ROOT/register/main/slightfile.toml secret -k "RESOLVER" -v "resolver_1"
slight -c $ROOT/register/main/slightfile.toml secret -k "STREAMER" -v "streamer_1"
slight -c $ROOT/register/main/slightfile.toml secret -k "STREAMER_API" -v "$STREAMER_API_URL"

slight -c $ROOT/metadata/main/slightfile.toml secret -k "INSTANCE" -v "metadata_1"
slight -c $ROOT/metadata/main/slightfile.toml secret -k "STORAGEINSTANCE" -v "storage_1"

slight -c $ROOT/locate/main/slightfile.toml secret -k "INSTANCE" -v "locator_1"
slight -c $ROOT/locate/main/slightfile.toml secret -k "STORAGEINSTANCE" -v "storage_1"

slight -c $ROOT/manifest/main/slightfile.toml secret -k "INSTANCE" -v "manifest_1"
slight -c $ROOT/resource/main/slightfile.toml secret -k "INSTANCE" -v "resource_1"
slight -c $ROOT/manifestverifier/main/slightfile.toml secret -k "INSTANCE" -v "manifestverifier_1"

slight -c $ROOT/resourceverifier/main/slightfile.toml secret -k "INSTANCE" -v "resourceverifier_1"

slight -c $ROOT/resolver/main/slightfile.toml secret -k "INSTANCE" -v "resolver_1"

slight -c $ROOT/resolver/main/slightfile.toml secret -k "DHTINSTANCE" -v "localhost:8004"

slight -c $ROOT/register/main/slightfile.toml secret -k "STORAGEINSTANCE" -v "storage_1"
slight -c $ROOT/manifest/main/slightfile.toml secret -k "STORAGEINSTANCE" -v "storage_1"
slight -c $ROOT/resource/main/slightfile.toml secret -k "STORAGEINSTANCE" -v "storage_1"

slight -c $ROOT/manifestverifier/main/slightfile.toml secret -k "MANIFESTINSTANCE" -v "manifest_1"

slight -c $ROOT/resourceverifier/main/slightfile.toml secret -k "RESOURCEINSTANCE" -v "resource_1"

slight -c $ROOT/resolver-rest-api/slightfile.toml secret -k "RESOLVERINSTANCE" -v "resolver_1"

slight -c $ROOT/metadataverifier/main/slightfile.toml secret -k "INSTANCE" -v "metadataverifier_1"
slight -c $ROOT/metadataverifier/main/slightfile.toml secret -k "METADATAINSTANCE" -v "metadata_1"

slight -c $ROOT/locateverifier/main/slightfile.toml secret -k "INSTANCE" -v "locatorverifier_1"
slight -c $ROOT/locateverifier/main/slightfile.toml secret -k "LOCATEINSTANCE" -v "locator_1"

slight -c $ROOT/streamer/slightfile.toml  secret -k "FILEPATH" -v "./streamer/static"
slight -c $ROOT/streamer/slightfile.toml  secret -k "STREAMER_API" -v "$STREAMER_API_URL"

./supervisor.sh $SLEEPTIME $ROOT/storage/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/storage.wasm &
./supervisor.sh $SLEEPTIME $ROOT/storage/main/slightfile2.toml $ROOT/target/wasm32-wasi/$VERSION/storage.wasm &

./supervisor.sh $SLEEPTIME $ROOT/register/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/register.wasm & 
./supervisor.sh $SLEEPTIME $ROOT/manifest/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/manifest.wasm &
./supervisor.sh $SLEEPTIME $ROOT/resource/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/resource.wasm &

./supervisor.sh $SLEEPTIME $ROOT/metadata/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/metadata.wasm & 

./supervisor.sh $SLEEPTIME $ROOT/locate/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/locate.wasm & 

./supervisor.sh $SLEEPTIME $ROOT/manifestverifier/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/manifestverifier.wasm &

./supervisor.sh $SLEEPTIME $ROOT/resourceverifier/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/resourceverifier.wasm &

./supervisor.sh $SLEEPTIME $ROOT/metadataverifier/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/metadataverifier.wasm &

./supervisor.sh $SLEEPTIME $ROOT/locateverifier/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/locateverifier.wasm &

./supervisor.sh $SLEEPTIME $ROOT/resolver/main/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/resolver.wasm &

./supervisor.sh $SLEEPTIME $ROOT/resolver-rest-api/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/resolver_rest_api.wasm &

./supervisor.sh $SLEEPTIME $ROOT/streamer/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/streamer.wasm &

#chord-dht-server localhost:8004 & 

#echo "Test register"
#slight -c $ROOT/register/test/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/testregister.wasm 

#ID: 352c0eedf4e42db2eea772a2d923cc97322589097ce1ba5928bc429aafcb38b6
#ID2: 34972210b8da7be9d10e153b9a03ff2d15258e8cfdb41fd3542a87c4ca8fd57f

#ID: bb6dac68789f17f02492d86c9c944883a6ee86c2314c6cd4fe5186e86fe70ef4
#ID2: bad4753c8d45ddb5246cbb9b562bc076d5490c9fde18b1cda21f2285dc03f2e0

#echo "Test manifest"
#slight -c $ROOT/manifest/test/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/testmanifest.wasm 
#echo "Test resource"
#slight -c $ROOT/resource/test/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/testresource.wasm 
#echo "Test manifestverifier"
#slight -c $ROOT/manifestverifier/test/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/testmanifestverifier.wasm 
#echo "Test resourceverifier"
#slight -c $ROOT/resourceverifier/test/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/testresourceverifier.wasm 
#echo "Test resolver"
#slight -c $ROOT/resolver/test/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/testresolver.wasm 

#slight -c $ROOT/locateverifier/test/slightfile.toml $ROOT/target/wasm32-wasi/$VERSION/testlocateverifier.wasm 