G=/srv/fast/amigo/scratch/g4
for g in "$@"; do
  echo "=== $g start $(date +%T)"
  docker run --rm --network none --cpus 2 --cpu-shares 256 -u $(id -u):$(id -g) -e HOME=/tmp -v $G/ws/$g:/w amigo-tools:3 nice -n 15 bash /w/g4_inner.sh
  echo "=== $g end $(date +%T) rc $?"
done
