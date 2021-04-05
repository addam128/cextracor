DATASET=./dataset/*.txt
NUMBER_OF_FILES=50

mkdir tmp

total_score=0
for file in $DATASET
do
  cp $file ./tmp/$(basename $file)
  cargo run ./tmp/$(basename $file)
done

for json_file in ./tmp/*.json
do
  file_score=$(python3 output_compare.py ./tmp/$(basename $json_file) ./dataset/$(basename $json_file))
  total_score=$(( $total_score + $file_score))
done

echo "scale=2 ; $total_score / $NUMBER_OF_FILES" | bc
