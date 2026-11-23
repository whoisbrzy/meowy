function v2(text){return [{type:17,components:[{type:10,content:String(text)}]}];}
async function reply(i,text){const components=v2(text);if(i.replied||i.deferred)return i.followUp({components,flags:32768});return i.reply({components,flags:32768});}
module.exports={v2,reply};