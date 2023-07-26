https://www.land.mlit.go.jp/webland_english/note.html
https://www.land.mlit.go.jp/webland/api.html
https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest

[Japanese version] https://www.land.mlit.go.jp/webland/api/TradeListSearch?<Parameter>
[English version] https://www.land.mlit.go.jp/webland_english/api/ TradeListSearch?<parameter>

from: year / quarter
YYYYN

to: 

area: Prefecture code NN

city: NNNNN

stations: NNNNN  0 prefix padded

output
Type	                transaction type	
Region	            district	
Municipality Code	City code	
Prefecture	        Name of prefectures	
Municipality	        City name	
District Name	    District name	
Trade Price	        Transaction price (gross price)	
Price Per Unit	Unit price per tsubo	Not included in English version
Floor Plan	Floor plan	
Area	area (square meters)	
Unit Price	Transaction price (per square meter)	
Land Shape	land shape	
Frontage	frontage	
Total Floor Area	Total floor area (square meters)	
Building Year	year of construction	
Structure	building structure	
Use	Usage	
Purpose	Future purpose of use	
Direction	Front road: direction	
Classification	Front road: type	
Breadth	Front road: Width (m)	
City Planning	city ​​planning	
Coverage ratio	Building coverage ratio (%)	
Floor Area Ratio	Floor-area ratio(%)	
period	time of transaction	
renovation	renovation	
Remarks	Circumstances of transactions, etc.



List of municipalities in prefectures acquisition API
By specifying the following parameters, you can obtain a list of municipality codes and municipality names within prefectures.
[Japanese version] https://www.land.mlit.go.jp/webland/api/CitySearch?<Parameter>
[English version] https://www.land.mlit.go.jp/webland_english/api/ CitySearch?<parameter>

<<parameter> >
parameter	content	remarks	Required
area	Prefecture code	The format is NN (2 digits)
NN … Prefecture code	○

<<Output>>
Output format: JSON format
The tags are as shown in the table below.
tag name	content
id	City code
name	City name

<<Usage example>>
https://www.land.mlit.go.jp/webland/api/CitySearch?area=13
Get a list of municipalities in Tokyo.
reference	List of prefecture codes
The list of prefecture codes and prefecture names used on this website is as follows.

