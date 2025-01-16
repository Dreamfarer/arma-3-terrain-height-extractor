private _stepSize = 5; // interval in meters
private _mapSize = worldsize; // automatic map size
"extractor" callExtension ["csv_init", ["height-data.csv"]]; 
for "_y" from 0 to _mapSize step _stepSize do {
    private _heightData = [0, _y, _stepSize];
    for "_x" from 0 to _mapSize step _stepSize do {
        _heightData pushBack getTerrainHeightASL [_x,_y];
    };
    "extractor" callExtension ["csv_write", ["height-data.csv", _heightData]];
    "_y";
};  
freeExtension "extractor";