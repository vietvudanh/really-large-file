#!/usr/bin/env bash
CWD="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# DATA_FILE=$CWD/data/itcont.txt
DATA_FILE=$CWD/data/by_date/itcont_2018_20020411_20170529.txt
LOG_FILE=log.log
BENCH_FILE=bench.log

# build
echo "run build"
bash $CWD/build.sh

# 
chmod -R +x dist

echo "list of script"
ls $CWD/dist | tr ' '

# all exe file will receive file from args
date >> $LOG_FILE
for exe in $(ls $CWD/dist/) ; do
    echo
	echo "running:: $exe" | tee -a $LOG_FILE
	bench=$(/usr/bin/time -f "real time::%Es" $CWD/dist/$exe $DATA_FILE | tee -a $LOG_FILE | grep "real time")
    echo $exe,$(date),$bench,$DATA_FILE >> $BENCH_FILE
done
