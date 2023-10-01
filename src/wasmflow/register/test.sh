../storage/target/release/storage --rpc --rpc-port 8082 &

wasmflow  invoke  ./register.yaml epubregister -- --filename="/childrens-literature5.epub"
wasmflow  invoke  ./register.yaml epubtest -- --filename="/sagrada-biblia-universidad-de-navarra_processed.epub"
wasmflow  invoke  ./register.yaml epubregister -- --filename="/BibliaJerusalen_processed.epub"

wasmflow serve ./register.yaml --rpc --rpc-port 8060 &

../rsstreamer/target/release/rsstreamer &

wasmflow  invoke   ./register.yaml epubresource -- --id="1a8103b5346a627f26f4670709b0e791ba7f744bcff9e65da3279efff9453f04" --path="EPUB/cover.xhtml" 

wasmflow  invoke ./register.yaml epublocator -- --id="1a8103b5346a627f26f4670709b0e791ba7f744bcff9e65da3279efff9453f04" --href="EPUB/s04.xhtml" --mediatype="text/html" --from="html body #pgepubid00492 #pgepubid00498 div" --to="html body #pgepubid00492 #pgepubid00501 h3"

id:92f13cb368b2fd0f45115588b3f0c9c3d6bfa8bfeda3083a33de0c8a02dcfa82
web+urs:self:7B0D0A20226872656622203A2220455055422F7330342E7868746D6C220D0A20226D656469617479706522203A22746578742F68746D6C22200D0A202266726F6D22203A202268746D6C20626F64792023706765707562696430303439322023706765707562696430303439382064697622200D0A2022746F22203A202268746D6C20626F6479202370676570756269643030343932202370676570756269643030353031206833220D0A7D

25101de0c87b8260ec7d0fb37fe6d6a781be1fcaa597e214876f3f6492fc21b5
3fae276a7c1e1b0dbcf4005f714d708c7374a7492dc005bb9b3523e5889eb657
075efed544f946e531ea298bd2ff17214c8a265f68b4ec225be64bdccb0ef376
curl http://localhost:8000/pub/1a8103b5346a627f26f4670709b0e791ba7f744bcff9e65da3279efff9453f04/manifest.json

https%3A%2F%2F8000-brown-lamprey-6p01dlro.ws-eu62.gitpod.io%2Fpub%2F1a8103b5346a627f26f4670709b0e791ba7f744bcff9e65da3279efff9453f04%2Fmanifest.json
