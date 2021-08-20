#!/bin/bash -e
# take from https://github.com/mindriot101/rust-etcd3

protodir=proto
sources=(api/etcdserverpb/rpc.proto api/mvccpb/kv.proto api/authpb/auth.proto)
branch="${1:-v3.5.0}"

mkdir -p $protodir

sedscript='
/gogoproto/d;
/annotations.proto/d;
/^import/s|".*/|"|;
/^\s*option (google\.api\.http) = {/ {
  :a;
  N;
  /};/!ba;
  d
}
'
echo $s

for source in "${sources[@]}"; do
  path="$protodir/$(basename $source)"
  mkdir -p $(dirname "$path")
  curl -f "https://raw.githubusercontent.com/etcd-io/etcd/$branch/$source" \
    | sed -e "$sedscript" > "$path"
done
