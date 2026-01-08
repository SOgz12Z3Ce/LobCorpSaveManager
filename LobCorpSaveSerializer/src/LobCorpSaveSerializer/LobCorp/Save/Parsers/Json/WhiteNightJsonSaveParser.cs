using System.Collections.Generic;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

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
		private static Dictionary<int, Dictionary<string, object>> ParseApostles(JObject ApostleList)
		{
			var result = new Dictionary<int, Dictionary<string, object>>();
			foreach (var kvp in ApostleList)
			{
				result[int.Parse(kvp.Key)] = ParseApostle(kvp.Value as JObject);
			}
			return result;
		}
		private static Dictionary<string, object> ParseApostle(JObject apostleListItem)
		{
			var result = new Dictionary<string, object>();

			Copy<int>(result, apostleListItem, "NameId");
			result["hairData"] = JsonSaveParserUtils.ParseHairStyle(apostleListItem["hairData"] as JObject);
			result["HairColor"] = JsonSaveParserUtils.ParseHairColor(apostleListItem["HairColor"] as JObject);
			Copy<long>(result, apostleListItem, "isntId");
			Copy<string>(result, apostleListItem, "Name");

			return result;
		}
	}
}
