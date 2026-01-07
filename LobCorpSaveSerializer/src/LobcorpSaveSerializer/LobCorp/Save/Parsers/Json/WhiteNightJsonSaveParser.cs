using System.Collections.Generic;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;
using WorkerSprite;

namespace LobCorp.Save.Parsers.Json
{
	public class WhiteNightJsonSaveParser : JsonSaveParserBase
	{
		public WhiteNightJsonSaveParser() : base(SaveType.WhiteNight)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			var data = new Dictionary<string, object>();

			Copy<int>(data, save, "genDay");
			Copy<int>(data, save, "apostleListCount");
			data["apostleList"] = ParseApostles(save["apostleList"] as JObject);
			
			return data;
		}
		private Dictionary<int, Dictionary<string, object>> ParseApostles(JObject ApostleList)
		{
			var result = new Dictionary<int, Dictionary<string, object>>();
			foreach (var kvp in ApostleList)
			{
				result[int.Parse(kvp.Key)] = ParseApostle(kvp.Value as JObject);
			}
			return result;
		}
		private Dictionary<string, object> ParseApostle(JObject apostleListItem)
		{
			var result = new Dictionary<string, object>();

			Copy<int>(result, apostleListItem, "NameId");
			result["hairData"] = ParseHairStyle(apostleListItem["hairData"] as JObject);
			result["HairColor"] = ParseHairColor(apostleListItem["HairColor"] as JObject);
			Copy<long>(result, apostleListItem, "isntId");
			Copy<string>(result, apostleListItem, "Name");

			return result;
		}
		private WorkerSpriteSaveData.Pair ParseHairStyle(JObject hairData)
		{
			return new WorkerSpriteSaveData.Pair
			{
				a = hairData["a"].Value<long>(),
				b = hairData["b"].Value<int>(),
			};
		}
		private WorkerSpriteSaveData.ColorData ParseHairColor(JObject hairData)
		{
			return new WorkerSpriteSaveData.ColorData
			{
				r = hairData["r"].Value<float>(),
				g = hairData["g"].Value<float>(),
				b = hairData["b"].Value<float>(),
				a = hairData["a"].Value<float>(),
			};
		}
	}
}
