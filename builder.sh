source $stdenv/setup

mkdir -p $out
cp -r $src/* $out/
cd $out

make ARCH=armv7 DEPS=${lua}/lib/ IDIR=${lua}/include
#make install
#make purge
echo $out
