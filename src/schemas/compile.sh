#!/bin/sh

cd "${MESON_SOURCE_ROOT}/${MESON_SUBDIR}";
echo "Executing in dir ${PWD}";

echo -e "\nFound flatbuffers schemas:";
find . -name "*.fbs" -type f -printf "%f\n";

echo -e "\nRemoving all old headers:";
rm -v *.rs;
flatc --rust *.fbs --gen-mutable &&

echo -e "\nCompiled headers:";
find . -name "*.rs" -type f -printf "%f\n";
