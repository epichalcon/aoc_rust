#/bin/bash
#
#
# Help

function help() {
    local help="Script para descargar el input de aoc para un dia y un año
    uso:
        ./download_day.sh [dia] [año]"
    echo $help
}

[ "$1" == "--help" ] || [ "$1" == "-h" ] && help && exit 0;

# Get the directory where teh script is located
ROOT_DIR=$(dirname -- ${BASH_SOURCE[0]})
# file shere session token is stored
ENV_FILE=${ROOT_DIR}/.env
# get session token
SESSION_TOKEN=$(grep SESSION "${ENV_FILE}" | sed s/SESSION=//g)


# ------------------------------------------------------------------

CURRENT_DAY=$(date +%d)
CURRENT_YEAR=$(date +%Y)

DAY=$1 # first argument
[ -z "${DAY}" ] && DAY=$CURRENT_DAY # default value 

YEAR=$2 # second argument
[ -z "${YEAR}" ] && YEAR=$CURRENT_YEAR # default value

OUTPUT_DIR="${ROOT_DIR}/src/inputs/y${YEAR}"

OUTPUT_FILE="${OUTPUT_DIR}/day${DAY}"

if [[ -e "${OUPTUT_FILE}" ]]
then
    echo "Fichero del dia ${DAY}, y año ${YEAR} existente"
    echo $OUPUT_FILE
    echo "rescribiendo fichero"
fi

[[ -d "${OUTPUT_DIR}" ]] && echo "Creando directorio : ${OUTPUT_DIR}"
mkdir -p "${OUTPUT_DIR}"


# ------------------------------------------------------------------
# curl command
INT_DAY=$(($DAY + 0))

CMD="curl --ssl-no-revoke"
CMD=${CMD}" -b session=${SESSION_TOKEN}"
CMD=${CMD}" https://adventofcode.com/${YEAR}/day/${INT_DAY}/input"
CMD=${CMD}" -o ${OUTPUT_FILE}"

${CMD}


# -------------------------------------------------------------------
# Editing y20XX.rs to refer to dayXX.rs

file=$ROOT_DIR/src/y$YEAR.rs

sed -i -e "s/\(mod day\).*/\1$DAY;/" \
    -e "s/\(day\).*\(::solve(&io::read(\).*\(,\).*/\1$DAY\2$YEAR\3$INT_DAY));/"   $file

# -------------------------------------------------------------------
# Creating dayXX.rs

cp ${ROOT_DIR}/src/template.rs ${ROOT_DIR}/src/y${YEAR}/day${DAY}.rs
