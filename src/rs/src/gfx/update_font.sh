export EXE=flatconvert-rs
export D=../../../../assets/fonts
#export EXE=flatconvert
${EXE}  -f $D/Waree.ttf -s 12 -r waree12.rs -p 2 
#${EXE}  -f Roboto-Light.ttf -s 12 -o roboto12.h -p 2 -r
${EXE}  -f $D/Roboto-Light.ttf -s 28 -r robotoLight28.rs -p 2 
${EXE}  -f $D/RobotoSlab-SemiBold.ttf  -s 48 -r robotoslab48.rs -p 2  -k "0123456789./AVWm"
