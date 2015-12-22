//no way in hell am I parsing json in freaking rust
"use strict";

const _ = require("lodash");
const data = require("./data/d12.json");

const fn = (data, acc) => {
    if(_.isObject(data) && !_.isArray(data) && _.contains(_.values(data), "red"))
        return 0;
    if(_.isNumber(data))
        return data;
    else if(_.isString(data))
        return 0;
    else
        return acc + _(data)
            .values()
            .map(val => fn(val, acc))
            .reduce((acc, val) => acc + val, 0);
};

console.log(fn(data, 0));
