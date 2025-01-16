"extractor" callExtension ["csv_init", ["height-data.csv"]]; 
"extractor" callExtension ["csv_write", ["height-data.csv", [0, 0, 5, -100, -102, -103]]];
"extractor" callExtension ["csv_write", ["height-data.csv", [0, 5, 5, -244, -245, -246]]]; 
freeExtension "extractor";