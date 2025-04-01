for file in $(find . -name '*.rs') ; do
    echo $file
    echo "\`\`\`rs"
    cat $file
    echo "\`\`\`"
    echo ""
done

