for dir in */; do  
    cd $dir  
    RUSTFLAGS=--cfg=web_sys_unstable_apis ./build.sh
    cd ..  
done