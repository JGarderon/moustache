#!/usr/bin/env bash

function display_title() {
  [[ $QUIET == "" ]] && echo "$1"
}

function display_result() {
  [[ $QUIET == "" ]] && echo "  --> $1"
}

function debug() {
  [ $DEBUG ] && echo "  >> $1"
}

function retrieve_part() {
  TESTFILE_PATH=$1
  TESTFILE_SEPARATOR=$2
  ITER=0
  FOUND=
  debug "try retrieve $3..."
  while read -r line
  do 
    debug "read line n°$ITER : $line"
    if [ "$line" == "$TESTFILE_SEPARATOR" ] 
    then
      FOUND=$ITER
      break
    fi
    ITER=$(expr $ITER + 1)
  done < <(tail -n "+$GLOBAL_ITER" "$TESTFILE_PATH")
  `echo declare -g "$3"_START=$GLOBAL_ITER`
  `echo declare -g "$3"_END=$ITER`
  GLOBAL_ITER=$(expr $GLOBAL_ITER + $ITER + 1)
  if [ "$FOUND" ]
  then
    debug "$3 found ($FOUND line[s]) ! next step..."
  else
    debug "$3 not found ! incorrect test file"
    exit 1
  fi
}

function execute_test () {
  # --- step 0
  EXEC_PATH=$1
  TESTFILE_PATH=$2
  debug "process file : '$TESTFILE_PATH'"
  TESTFILE_SEPARATOR=`head -n 1 "$TESTFILE_PATH"`
  debug "found separator : '$TESTFILE_SEPARATOR'"
  GLOBAL_ITER=2
  # --- step 1
  retrieve_part $TESTFILE_PATH $TESTFILE_SEPARATOR "CONF"
  TEST_COMMAND=`tail -n "+$CONF_START" "$TESTFILE_PATH" | head -n "$CONF_END" | tr -d "\n"`
  # --- step 2
  retrieve_part $TESTFILE_PATH $TESTFILE_SEPARATOR "SOURCE_IN"
  # --- step 3
  retrieve_part $TESTFILE_PATH $TESTFILE_SEPARATOR "SOURCE_OUT"
  # --- step 4
  display_result "process command : '$TEST_COMMAND'"
  RESULT="`tail -n "+$SOURCE_IN_START" "$TESTFILE_PATH" | head -n "$SOURCE_IN_END" | sh -c "alias moustache=$EXEC_PATH; $TEST_COMMAND 2>&1"`" 
  RETURNCODE="$?"
  if [ "$RETURNCODE" != 0 ]
  then
    return 1
  fi
  RESULT=$(echo $RESULT | LC_ALL=C.UTF8 sed -E "s/\x1B\[[\x30-\x3F]*[\x20-\x20F]*[\x40-\x7E]//g")
  EXPECTED_RESULT="`tail -n "+$SOURCE_OUT_START" "$TESTFILE_PATH" | head -n "$SOURCE_OUT_END"`" 
  EXPECTED_RESULT=$(echo $EXPECTED_RESULT | LC_ALL=C.UTF8 sed -E "s/\x1B\[[\x30-\x3F]*[\x20-\x20F]*[\x40-\x7E]//g")
  debug "return : !EOF"
  debug "$RESULT"
  debug "EOF!"
  debug "expected : !EOF"
  debug "$EXPECTED_RESULT"
  debug "EOF!"
  if [ "$EXPECTED_RESULT" = "$RESULT" ]
  then
    debug "expected result found"
    return 0
  else
    debug "expected result not found"
    return 1
  fi
}

function execute_tests() {
  if [[ $2 == "" ]]
  then 
    display_title "tests path not defined"
    exit 0
  fi
  for path in `ls $2`
  do
    display_title "Found '$path' test file"
    execute_test "$1" "$path"
    RETURNCODE="$?"
    if [ "$RETURNCODE" == 0 ]
    then
      display_result "test passed !"
    else
      display_result "test failed !"
      KO=$(expr $KO + 1)
    fi
    SUM=$(expr $SUM + 1)
  done
}

if [ "$DEBUG" == "0" ]
then
  DEBUG=
fi

SUM=0
KO=0
execute_tests "$1" "$2"
if [ $KO == 0 ]
then 
  display_title "All tests passed ($SUM test[s])"
  exit 0
else
  display_title "$KO test[s] failed (/ $SUM test[s])"
  exit 1
fi

