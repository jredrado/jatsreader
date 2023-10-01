namespace "resourceverifier"

type Inputs {
  id: bytes
  path: string
  proofs: bytes
}

type Outputs {
  contenttype: string
  data: bytes
}

type Config {

}