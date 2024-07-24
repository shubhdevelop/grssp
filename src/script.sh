file_path="example.txt"

for i in $(seq 1 100000); do
    echo "Hello this side is bigot" >> "$file_path"
done