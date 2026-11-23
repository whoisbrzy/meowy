require('dotenv').config();
const { Client, GatewayIntentBits, REST, Routes } = require('discord.js');
const { commands } = require('./commands');
const client = new Client({ intents: [GatewayIntentBits.Guilds] });
const token = process.env.DISCORD_TOKEN;
if (!token) throw new Error('DISCORD_TOKEN is required');
const rest = new REST({ version: '10' }).setToken(token);
(async()=>{
 await rest.put(Routes.applicationCommands(process.env.CLIENT_ID || (await client.login(token)).user.id), { body: commands.map(c=>c.data.toJSON()) });
 console.log('Meowy command registration complete.');
})().catch(console.error);
client.on('ready',()=>console.log(`Meowy online as ${client.user.tag}`));
client.on('interactionCreate', async interaction=>{
 if(!interaction.isChatInputCommand()) return;
 const command=commands.find(c=>c.data.name===interaction.commandName);
 if(!command) return;
 try { await command.execute(interaction); } catch(e){console.error(e);const msg=v2('An error occurred while running this command.');if(interaction.replied||interaction.deferred) await interaction.followUp({components:msg,flags:32768});else await interaction.reply({components:msg,flags:32768});}
});
client.login(token);
function v2(text){return [{type:17,components:[{type:10,content:String(text)}]}];}
module.exports.v2=v2;